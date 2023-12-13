using BreadLang.Tokens;

namespace BreadLang.Parser.Nodes.InstructionNodes;

public class JoNode : Node
{
    public override void Populate(Parser parser)
    {
        if (parser.Check(TokenType.Number))
        {
            PopulateAndAdd(new NumberNode(NumberNode.Type.Immediate16), parser);
        }
    }

    public override byte[] Compile()
    {
        throw new NotImplementedException();
    }

    public override string ToString()
    {
        return "JO";
    }
}