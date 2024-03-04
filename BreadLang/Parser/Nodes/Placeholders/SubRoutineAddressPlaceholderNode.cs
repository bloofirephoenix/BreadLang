using BreadLang.Compiling;
using BreadLang.Tokens;

namespace BreadLang.Parser.Nodes.Placeholders;

public class SubRoutineAddressPlaceholderNode : PlaceholderNode
{
	public override void Compile(Compiler compiler)
    {
        if (!compiler.Placeholders.ContainsKey(Token!.Lexeme))
        {
            ErrorHandler.Instance!.Error(Token, $"Could not find {Token.Lexeme}");
            return;
        }
        compiler.Write(compiler.Placeholders[Token.Lexeme]);
    }

    public override string ToString()
    {
        return $"SubRoutineAddressPlaceholderNode(name={Token!.Lexeme})";
    }

    public override int GetSize()
    {
        return 2;
    }
}
