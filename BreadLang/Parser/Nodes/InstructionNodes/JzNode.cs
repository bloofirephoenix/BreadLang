using BreadLang.Compiling;
using BreadLang.Tokens;

namespace BreadLang.Parser.Nodes.InstructionNodes;

public class JzNode : Node
{
    public override void Populate(Parser parser)
    {
        PopulateAndAdd(new RegisterNode(), parser);

        if (parser.Check(TokenType.Identifier))
            PopulateAndAdd(new PlaceholderNode(), parser);
    }

    public override void Compile(Compiler compiler)
    {
        var reg = compiler.GetRegister(((RegisterNode)Children[0]).Register);
        if (Children.Count == 0)
        {
            compiler.WriteFirstByte(OpCodes.Jz, false, reg);
        }
        else
        {
            compiler.WriteFirstByte(OpCodes.Jz, true, reg);
            Children[0].Compile(compiler);
        }
    }

    public override string ToString()
    {
        return "JZ";
    }

    public override int GetSize()
    {
        return 1 + (Children.Count > 0 ? 2 : 0);
    }
}