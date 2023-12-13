using BreadLang.Tokens;

namespace BreadLang.Parser.Nodes.InstructionNodes;

public class AddNode : Node
{
    public override void Populate(Parser parser)
    {
        PopulateAndAdd(new RegisterNode(), parser);

        parser.Expect(TokenType.Comma);

        if (parser.Check(TokenType.Register))
            PopulateAndAdd(new RegisterNode(), parser);
        else
            PopulateAndAdd(new NumberNode(NumberNode.Type.Immediate8), parser);
    }

    public override byte[] Compile()
    {
        throw new NotImplementedException();
    }

    public override string ToString()
    {
        return "ADD";
    }
}