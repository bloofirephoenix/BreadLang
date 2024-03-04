using BreadLang.Compiling;
using BreadLang.Parser.Nodes.Placeholders;
using BreadLang.Tokens;

namespace BreadLang.Parser.Nodes.InstructionNodes;

public class JoNode : Node
{
    public override void Populate(Parser parser)
    {
        if (parser.Check(TokenType.Identifier))
        {
            PopulateAndAdd(new SubRoutineAddressPlaceholderNode(), parser);
        }
    }

    public override void Compile(Compiler compiler)
    {
        if (Children.Count == 0)
        {
            compiler.WriteFirstByte(OpCodes.Jo, false, null);
        }
        else
        {
            compiler.WriteFirstByte(OpCodes.Jo, true, null);
            Children[0].Compile(compiler);
        }
    }

    public override string ToString()
    {
        return "JO";
    }

    public override int GetSize()
    {
        return Children.Count > 0 ? 3 : 1;
    }
}