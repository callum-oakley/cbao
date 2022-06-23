#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "common.h"
#include "compiler.h"
#include "scanner.h"

#ifdef DEBUG_PRINT_CODE
#include "debug.h"
#endif

typedef struct {
    Token current;
    Token previous;
    bool error;
} Parser;

typedef struct {
    Token name;
    int depth;
} Variable;

// TODO I think for bao scope_depth is equivalent to variable_count, since let
// always binds a single variable and introduces a new scope. Maybe this isn't
// the case for functions though?
typedef struct {
    Variable variables[UINT8_COUNT];
    int variable_count;
    int scope_depth;
} Compiler;

Parser parser;
Compiler* current = NULL;
Chunk* compiling_chunk;

static Chunk* current_chunk() {
    return compiling_chunk;
}

static void error_at(Token* token, const char* message) {
    if (parser.error) return;
    fprintf(stderr, "[line %d] Error", token->line);

    if (token->type == TOKEN_EOF) {
        fprintf(stderr, " at end");
    } else if (token->type == TOKEN_ERROR) {
        // noop
    } else {
        fprintf(stderr, " at '%.*s'", token->length, token->start);
    }

    fprintf(stderr, ": %s\n", message);
    parser.error = true;
}

static void error(const char* message) {
    error_at(&parser.previous, message);
}

static void error_at_current(const char* message) {
    error_at(&parser.current, message);
}

static void advance() {
    parser.previous = parser.current;

    for (;;) {
        parser.current = scan_token();
        if (parser.current.type != TOKEN_ERROR) break;

        error_at_current(parser.current.start);
    }
}

static void consume(TokenType type, const char* message) {
    if (parser.current.type == type) {
        advance();
        return;
    }

    error_at_current(message);
}

static void emit_byte(uint8_t byte) {
    write_chunk(current_chunk(), byte, parser.previous.line);
}

static void emit_bytes(uint8_t byte0, uint8_t byte1) {
    emit_byte(byte0);
    emit_byte(byte1);
}

static void emit_return() {
    emit_byte(OP_RETURN);
}

static uint8_t make_constant(Value value) {
    int constant = add_constant(current_chunk(), value);
    if (constant > UINT8_MAX) {
        error("Too many constants in one chunk.");
        return 0;
    }

    return (uint8_t)constant;
}

static void emit_constant(Value value) {
    emit_bytes(OP_CONSTANT, make_constant(value));
}

static void init_compiler(Compiler* compiler) {
    compiler->variable_count = 0;
    compiler->scope_depth = 0;
    current = compiler;
}

static void end_compiler() {
    emit_return();
#ifdef DEBUG_PRINT_CODE
    if (!parser.error) {
        disassemble_chunk(current_chunk(), "code");
    }
#endif
}

static void begin_scope() {
    current->scope_depth++;
}

static void end_scope() {
    current->scope_depth--;
    while (
        current->variable_count > 0 &&
        current->variables[current->variable_count - 1].depth > current->scope_depth
    ) {
        emit_byte(OP_PUSH_DOWN);
        current->variable_count--;
    }
}

static bool is_symbol(const char* symbol, int length) {
    return parser.previous.length == length &&
        memcmp(parser.previous.start, symbol, length) == 0;
}

static bool identifiers_equal(Token* a, Token* b) {
    return a->length == b->length &&
        memcmp(a->start, b->start, a->length) == 0;
}

static void add_variable(Token name) {
    if (current->variable_count == UINT8_COUNT) {
        error("Too many variables.");
        return;
    }

    Variable* variable = &current->variables[current->variable_count++];
    variable->name = name;
    variable->depth = current->scope_depth;
}

static void expression();

static void list() {
    // hack that handles primitive ops only
    advance();
    if (is_symbol("not", 3)) {
        expression();
        emit_byte(OP_NOT);
    } else if (is_symbol("+", 1)) {
        expression();
        expression();
        emit_byte(OP_ADD);
    } else if (is_symbol("-", 1)) {
        expression();
        if (parser.current.type == TOKEN_RIGHT_PAREN) {
            emit_byte(OP_NEGATE);
        } else {
            expression();
            emit_byte(OP_SUBTRACT);
        }
    } else if (is_symbol("*", 1)) {
        expression();
        expression();
        emit_byte(OP_MULTIPLY);
    } else if (is_symbol("/", 1)) {
        expression();
        expression();
        emit_byte(OP_DIVIDE);
    } else if (is_symbol("=", 1)) {
        expression();
        expression();
        emit_byte(OP_EQUAL);
    } else if (is_symbol("not=", 4)) {
        expression();
        expression();
        emit_byte(OP_NOT_EQUAL);
    } else if (is_symbol("<", 1)) {
        expression();
        expression();
        emit_byte(OP_LESS);
    } else if (is_symbol(">", 1)) {
        expression();
        expression();
        emit_byte(OP_GREATER);
    } else if (is_symbol("<=", 2)) {
        expression();
        expression();
        emit_byte(OP_LESS_EQUAL);
    } else if (is_symbol(">=", 2)) {
        expression();
        expression();
        emit_byte(OP_GREATER_EQUAL);
    } else if (is_symbol("print", 5)) {
        expression();
        emit_byte(OP_PRINT);
    } else {
        // TODO
    }
    consume(TOKEN_RIGHT_PAREN, "Expect ')'");
}

static void number() {
    emit_constant(NUMBER_VAL(strtol(parser.previous.start, NULL, 10)));
}

static void string() {
    emit_constant(OBJ_VAL(copy_string(parser.previous.start + 1, parser.previous.length - 2)));
}

static int resolve_variable(Compiler* compiler, Token* name) {
    for (int i = compiler->variable_count - 1; i >= 0; i--) {
        Variable* variable = &compiler->variables[i];
        if (identifiers_equal(name, &variable->name)) {
            return i;
        }
    }

    return -1;
}

static void symbol() {
    if (is_symbol("nil", 3)) {
        emit_byte(OP_NIL);
    } else if (is_symbol("true", 4)) {
        emit_byte(OP_TRUE);
    } else if (is_symbol("false", 5)) {
        emit_byte(OP_FALSE);
    } else if (is_symbol("let", 3)) {
        advance();
        Token name = parser.previous;
        expression();
        begin_scope();
        // TODO Do we need to declare and then initialize like in the book?
        // Defining variables in terms of their previous value is fine for
        // non-functions, but might be a pain when we want to write recursive
        // functions.
        add_variable(name);
        expression();
        end_scope();
    } else if (is_symbol("do", 2)) {
        expression();
        emit_byte(OP_POP);
        expression();
    } else {
        int slot = resolve_variable(current, &parser.previous);
        if (slot == -1) {
            error("Can't resolve variable.");
        } else {
            emit_bytes(OP_GET_VARIABLE, (uint8_t)slot);
        }
    }
}

static void expression() {
    if (parser.error) return;
    advance();
    switch (parser.previous.type) {
        case TOKEN_NUMBER: return number();
        case TOKEN_LEFT_PAREN: return list();
        case TOKEN_SYMBOL: return symbol();
        case TOKEN_STRING: return string();
        default: error("Unexpected token.");
    }
}

bool compile(const char* source, Chunk* chunk) {
    init_scanner(source);
    Compiler compiler;
    init_compiler(&compiler);
    compiling_chunk = chunk;
    parser.error = false;
    advance();
    expression();
    if (parser.current.type != TOKEN_EOF) {
        error_at_current("Unexpected token.");
    }
    emit_byte(OP_PRINT);
    end_compiler();
    return !parser.error;
}
