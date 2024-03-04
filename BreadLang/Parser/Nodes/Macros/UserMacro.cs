using BreadLang.Parser.Nodes;
using BreadLang.Tokens;

namespace BreadLang.Macros;

public class UserMacro(ProgramNode programNode, List<string> arguments) : IMacro
{
	private List<Token> _tokens = new();
	private List<string> _arguments = arguments;

	public void ReadTokens(Parser.Parser parser) 
	{
		parser.SkipNewLines();
		var indent = parser.Expect(TokenType.Indent);

		if (!programNode.CheckIndent(indent))
			ErrorHandler.Instance!.Error(indent, "Invalid indent");
		
		while (!parser.IsAtEnd())
		{
			if (parser.Check(TokenType.NewLine)) 
			{
				parser.SkipNewLines();
				if (!programNode.CheckIndent(parser.Peek()))
					return;
				
				parser.Advance(); // advance past indent token
			}
			var token = parser.Advance();
			
			_tokens.Add(token);
		}
	}

	public List<Token> GetTokens(List<Token> arguments) 
	{
		List<Token> tokens = new();

		foreach (var token in _tokens) 
		{
			if (token.Type == TokenType.Identifier)
			{
				if (_arguments.Contains(token.Lexeme)) 
				{
					// replace identifier token with argument
					tokens.Add(arguments[_arguments.IndexOf(token.Lexeme)]);
					continue;
				}
			}

			tokens.Add(token);
		}

		return tokens;
	}
}
