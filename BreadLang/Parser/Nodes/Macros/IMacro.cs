using BreadLang.Tokens;

namespace BreadLang.Macros;

public interface IMacro
{
	public List<Token> GetTokens(List<Token> arguments);
}
