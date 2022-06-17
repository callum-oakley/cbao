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

Parser parser;
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

static void end_compiler() {
    emit_return();
#ifdef DEBUG_PRINT_CODE
    if (!parser.error) {
        disassemble_chunk(current_chunk(), "code");
    }
#endif
}

static bool is_symbol(const char* symbol) {
    return strncmp(parser.previous.start, symbol, parser.previous.length) == 0;
}

static void expression();

static void list() {
    // hack that handles primitive ops only
    advance();
    if (is_symbol("not")) {
        expression();
        emit_byte(OP_NOT);
    } else if (is_symbol("+")) {
        expression();
        expression();
        emit_byte(OP_ADD);
    } else if (is_symbol("-")) {
        expression();
        if (parser.current.type == TOKEN_RIGHT_PAREN) {
            emit_byte(OP_NEGATE);
        } else {
            expression();
            emit_byte(OP_SUBTRACT);
        }
    } else if (is_symbol("*")) {
        expression();
        expression();
        emit_byte(OP_MULTIPLY);
    } else if (is_symbol("/")) {
        expression();
        expression();
        emit_byte(OP_DIVIDE);
    } else if (is_symbol("=")) {
        expression();
        expression();
        emit_byte(OP_EQUAL);
    } else if (is_symbol("not=")) {
        expression();
        expression();
        emit_byte(OP_NOT_EQUAL);
    } else if (is_symbol("<")) {
        expression();
        expression();
        emit_byte(OP_LESS);
    } else if (is_symbol(">")) {
        expression();
        expression();
        emit_byte(OP_GREATER);
    } else if (is_symbol("<=")) {
        expression();
        expression();
        emit_byte(OP_LESS_EQUAL);
    } else if (is_symbol(">=")) {
        expression();
        expression();
        emit_byte(OP_GREATER_EQUAL);
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

static void symbol() {
    if (is_symbol("nil")) {
        emit_byte(OP_NIL);
    } else if (is_symbol("true")) {
        emit_byte(OP_TRUE);
    } else if (is_symbol("false")) {
        emit_byte(OP_FALSE);
    } else {
        // TODO
    }
}

static void expression() {
    if (parser.error) return;
    advance();
    switch (parser.previous.type) {
        case TOKEN_NUMBER: return number();
        case TOKEN_LEFT_PAREN: return list();
        case TOKEN_SYMBOL: return symbol();
        default: error("Unexpected token.");
    }
}

bool compile(const char* source, Chunk* chunk) {
    init_scanner(source);
    compiling_chunk = chunk;
    parser.error = false;
    advance();
    expression();
    consume(TOKEN_EOF, "Expect end of expression.");
    end_compiler();
    return !parser.error;
}
