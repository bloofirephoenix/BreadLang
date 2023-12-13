using BreadLang.Tokens;

namespace BreadLang.Parser.Nodes.InstructionNodes;

public class JzNode : Node
{
    public override void Populate(Parser parser)
    {
        var register = new RegisterNode();
        register.Populate(parser);
        Children.Add(register);

        if (parser.Peek().Type == TokenType.Number)
            PopulateAndAdd(new NumberNode(NumberNode.Type.Immediate16), parser);
    }

    public override byte[] Compile()
    {
        throw new NotImplementedException();
    }

    public override string ToString()
    {
        return "JZ";
    }
}