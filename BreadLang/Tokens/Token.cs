namespace BreadLang.Tokens;

public enum TokenType
{
    // literals
    Identifier,
    Number,

    // 1 char tokens
    Comma, Colon, OpenParenthesis, CloseParenthesis,

    Indent, NewLine,

    // keywords
    Macro, Include, NextAddr,

    // Instructions
    Nop, Lw, Sw, Mw, Push, Pop, Lda, Jmp, Jz, Jo, Add, Sub, Tel, Out, Hlt,

    // Registers
    Register,

    Eof
}

public record Token(TokenType Type, string Lexeme, object? Literal, int Line)
{
    public override string ToString()
    {
        return $"Token(type={Type}, lexeme={Lexeme.Replace("\n", @"\n").Replace("\r", @"\r")}, literal={Literal}, line={Line})";
    }
}