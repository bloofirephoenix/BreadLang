using BreadLang.Compiling;
using BreadLang.Tokens;

namespace BreadLang.Parser.Nodes.InstructionNodes;

public class LwNode : Node
{
    public override void Populate(Parser parser)
    {
        PopulateAndAdd(new RegisterNode(), parser);

        if (parser.Check(TokenType.Comma))
        {
            // there is an immediate value
            parser.Advance(); // comma

            PopulateAndAdd(new NumberNode(NumberNode.Type.Immediate16), parser);
        }

    }

    public override void Compile(Compiler compiler)
    {

    }

    public override int GetSize()
    {
        return 1 + (Children.Count > 0 ? 2 : 0);
    }

    public override string ToString()
    {
        return "LW";
    }
}