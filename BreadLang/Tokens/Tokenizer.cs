using System;

namespace BreadLang.Tokens;

public class Tokenizer(string source)
{
    private static Dictionary<string, TokenType> keywords = new()
    {
        { "@macro", TokenType.Macro },
        { "@include", TokenType.Include },

        { "NOP", TokenType.Nop },
        { "LW", TokenType.Lw },
        { "SW", TokenType.Sw },
        { "MW", TokenType.Mw },
        { "PUSH", TokenType.Push },
        { "POP", TokenType.Pop },
        { "LDA", TokenType.Lda },
        { "JMP", TokenType.Jmp },
        { "JZ", TokenType.Jz },
        { "JO", TokenType.Jo },
        { "ADD", TokenType.Add },
        { "SUB", TokenType.Sub },
        { "OUT", TokenType.Out },
        { "HLT", TokenType.Hlt },

        { "A", TokenType.Register },
        { "B", TokenType.Register },
        { "L", TokenType.Register },
        { "H", TokenType.Register },
    };

    private int _start = 0;
    private int _current = 0;
    private int _line = 1;

    private char _char;

    private List<Token> _tokens = new();

    public List<Token> ScanTokens()
    {
        while (!IsAtEnd())
        {
            _start = _current;
            ScanToken();
        }

        _tokens.Add(new Token(TokenType.Eof, "", null, _line));
        return _tokens;
    }

    private void ScanToken()
    {
        Advance();
        switch (_char)
        {
            case ':': AddToken(TokenType.Colon); break;
            case ',': AddToken(TokenType.Comma); break;

            case ';': // comments
                while (Peek() != '\n' && !IsAtEnd()) 
                    Advance();
                break;

            case ' ':
            case '\t':
                if (_tokens.Count > 0 && _tokens.Last().Type != TokenType.NewLine)
                    break; // ignore whitespace in the middle of line

                // add indent token if its the start of the line
                while (char.IsWhiteSpace(Peek()))
                    Advance();

                AddToken(TokenType.Indent);
                break;

            case '\r':
                // ignore
                break;

            case '\n':
                _line++;
                AddToken(TokenType.NewLine);
                break;
            default:
                if (IsDigit(_char))
                {
                    Number();
                }
                else if (IsAlpha(_char))
                {
                    Identifier();
                }
                else
                {
                    ErrorHandler.Instance!.Error(_line, _char.ToString(), "Unexpected character");
                }
                break;
        }
    }

    private void Identifier()
    {
        while (IsAlphaNumeric(Peek())) Advance();

        string text = source.Substring(_start, _current - _start);
        if (keywords.ContainsKey(text))
        {
            AddToken(keywords[text]);
        }
        else
        {
            AddToken(TokenType.Identifier);
        }
    }

    private void Number()
    {
        if (_char == '0')
        {
            switch (Peek())
            {
                case 'x':
                case 'X':
                    Advance();
                    // hexadecimal
                    while (IsDigit(Peek()) || Peek() is >= 'A' and <= 'F' || Peek() is >= 'a' and <= 'f')
                        Advance();
                    
                    break;
                case 'b':
                case 'B':
                    Advance();
                    // binary
                    while (Peek() is '0' or '1' or '_')
                        Advance();
                    break;
            }
        }

        while (IsDigit(Peek()) || Peek() == '_')
            Advance();

        AddToken(TokenType.Number);
    }

    private bool IsDigit(char c)
    {
        return c is >= '0' and <= '9';
    }

    private bool IsAlpha(char c)
    {
        return (c is (>= 'a' and <= 'z') or (>= 'A' and <= 'Z') or '_');
    }

    private bool IsAlphaNumeric(char c)
    {
        return IsAlpha(c) || IsDigit(c);
    }

    private char Peek()
    {
        if (IsAtEnd()) return '\0';
        return source[_current];
    }

    private char PeekNext()
    {
        if (_current + 1 >= source.Length) return '\0';
        return source[_current + 1];
    }

    private bool Match(char expected)
    {
        if (IsAtEnd()) return false;
        if (source[_current] != expected) return false;

        _current++;
        return true;
    }

    private void Advance()
    {
        _char = source[_current++];
    }

    private void AddToken(TokenType type, object? literal = null)
    {
        string text = source.Substring(_start, _current - _start);
        _tokens.Add(new Token(type, text, literal, _line));
    }

    private bool IsAtEnd()
    {
        return _current >= source.Length;
    }
}