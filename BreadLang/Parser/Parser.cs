﻿using BreadLang.Tokens;

namespace BreadLang.Parser;

public class Parser(List<Token> tokens)
{
    private int _current = 0;

    public Token Peek()
    {
        if (IsAtEnd())
            return tokens.Last();

        return tokens[_current];
    }

    public void SkipNewLines()
    {
        while (Peek().Type == TokenType.NewLine)
            Advance();
    }

    public Token Expect(TokenType type)
    {
        if (Peek().Type != type)
        {
            ErrorHandler.Instance!.Error(Peek(), $"Expected {type} found {Peek().Type}");
        }

        return Advance();
    }

    public bool Check(TokenType type)
    {
        return Peek().Type == type;
    }

    public Token Advance()
    {
        if (IsAtEnd())
            return tokens.Last();

        return tokens[_current++];
    }

    public Token Current()
    {
        if (_current > 0)
            return tokens[_current - 1];
        else
            return tokens.First();
    }

    public bool IsAtEnd()
    {
        return _current >= tokens.Count;
    }
}