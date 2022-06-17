#include <stdio.h>
#include <string.h>

#include "common.h"
#include "scanner.h"

// TODO why is this here and not in the header? or: why is the VM struct
// defined in the header?
typedef struct {
  const char* start;
  const char* current;
  int line;
} Scanner;

Scanner scanner;

void init_scanner(const char* source) {
    scanner.start = source;
    scanner.current = source;
    scanner.line = 1;
}

static bool is_digit(char c) {
    return c >= '0' && c <= '9';
}

static bool is_symbolic(char c) {
    return c != ' ' && c != '\r' && c != '\t' && c != '\n' && c != ';'
        && c != '(' && c != ')' && c != '"' && c != '\0';
}

static bool is_at_end() {
    return *scanner.current == '\0';
}

static char advance() {
    scanner.current++;
    return scanner.current[-1];
}

static char peek() {
    return *scanner.current;
}

static char peek_next() {
    if (is_at_end()) return '\0';
    return scanner.current[1];
}

static Token make_token(TokenType type) {
    Token token;
    token.type = type;
    token.start = scanner.start;
    token.length = (int)(scanner.current - scanner.start);
    token.line = scanner.line;
    return token;
}

static Token error_token(const char* message) {
    Token token;
    token.type = TOKEN_ERROR;
    token.start = message;
    token.length = (int)strlen(message);
    token.line = scanner.line;
    return token;
}

static void skip_whitespace() {
    for (;;) {
        char c = peek();
        switch (c) {
            case ' ':
            case '\r':
            case '\t':
                advance();
                break;
            case '\n':
                scanner.line++;
                advance();
                break;
            case ';':
                while (peek() != '\n' && !is_at_end()) advance();
                break;
            default:
                return;
        }
    }
}

static Token number() {
    while (is_digit(peek())) advance();
    return make_token(TOKEN_NUMBER);
}

static Token symbol() {
    while (is_symbolic(peek())) advance();
    return make_token(TOKEN_SYMBOL);
}

static Token string() {
    while (peek() != '"' && !is_at_end()) {
        if (peek() == '\n') scanner.line++;
        advance();
    }
    if (is_at_end()) return error_token("Unterminated string.");
    advance(); // consume the closing quote
    return make_token(TOKEN_STRING);
}

Token scan_token() {
    skip_whitespace();
    scanner.start = scanner.current;

    if (is_at_end()) return make_token(TOKEN_EOF);

    char c = advance();
    if (is_digit(c) || c == '-' && is_digit(peek())) return number();

    switch (c) {
        case '(': return make_token(TOKEN_LEFT_PAREN);
        case ')': return make_token(TOKEN_RIGHT_PAREN);
        case '.': return make_token(TOKEN_DOT);
        case '"': return string();
        default: return symbol();
    }
}
