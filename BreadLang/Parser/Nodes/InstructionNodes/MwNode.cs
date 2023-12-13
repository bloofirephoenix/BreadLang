using BreadLang.Tokens;

namespace BreadLang.Parser.Nodes.InstructionNodes;

public class MwNode : Node
{
    public override void Populate(Parser parser)
    {
        var rNode = new RegisterNode();
        rNode.Populate(parser);
        Children.Add(rNode);

        parser.Expect(TokenType.Comma);

        if (parser.Peek().Type == TokenType.Register)
        {
            var r2Node = new RegisterNode();
            r2Node.Populate(parser);
            Children.Add(r2Node);
        }
        else
        {
            var number = new NumberNode(NumberNode.Type.Immediate8);
            number.Populate(parser);
            Children.Add(number);
        }
    }

    public override byte[] Compile()
    {
        throw new NotImplementedException();
    }

    public override string ToString()
    {
        return "Mw";
    }
}