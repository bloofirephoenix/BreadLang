using BreadLang.Tokens;

namespace BreadLang.Parser;

public class Parser(List<Token> tokens)
{
	private List<Token> _tokens = tokens;
	private int _current;

	public Token Peek()
	{
		if (IsAtEnd())
			return _tokens.Last();

		return _tokens[_current];
	}

	public Token PeekNext()
	{
		if (_current + 1 >= _tokens.Count)
			return _tokens.Last();

		return _tokens[_current + 1];
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

	public bool ExpectPeek(TokenType type)
	{
		if (Peek().Type != type)
		{
			ErrorHandler.Instance!.Error(Peek(), $"Expected {type} found {Peek().Type}");
		}

		return true;
	}

	public Token Advance()
	{
		if (IsAtEnd())
			return _tokens.Last();

		return _tokens[_current++];
	}

	public Token Current()
	{
		if (_current > 0)
			return _tokens[_current - 1];
		else
			return _tokens.First();
	}

	public bool IsAtEnd()
	{
		return _current >= _tokens.Count;
	}

	public void InsertTokens(List<Token> tokens) 
	{
		_tokens.InsertRange(_current, tokens);
	}
}