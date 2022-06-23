#ifndef bao_scanner_h
#define bao_scanner_h

typedef enum {
    TOKEN_LEFT_PAREN, TOKEN_RIGHT_PAREN,
    TOKEN_NUMBER, TOKEN_SYMBOL, TOKEN_STRING,
    TOKEN_ERROR, TOKEN_EOF,
} TokenType;

typedef struct {
    TokenType type;
    const char* start;
    int length;
    int line;
} Token;

void init_scanner(const char* source);
Token scan_token();

#endif
