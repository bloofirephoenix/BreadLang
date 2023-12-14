using BreadLang.Compiling;
using BreadLang.Tokens;

namespace BreadLang.Parser.Nodes.InstructionNodes;

public class PushNode : Node
{
    public override void Populate(Parser parser)
    {
        Node node;
        if (parser.Peek().Type == TokenType.Number)
            node = new NumberNode(NumberNode.Type.Immediate8);
        else if (parser.Check(TokenType.Register))
            node = new RegisterNode();
        else
        {
            ErrorHandler.Instance!.Error(parser.Current(), "Expected number or register");
            return;
        }

        node.Populate(parser);
        Children.Add(node);
    }

    public override void Compile(Compiler compiler)
    {
        var node = Children[0];

        if (node is RegisterNode reg)
        {
            compiler.WriteFirstByte(OpCodes.Push, false, compiler.GetRegister(reg.Register));
            return;
        }

        if (node is NumberNode num)
        {
            compiler.WriteFirstByte(OpCodes.Push, true, null);
            compiler.Write((byte)num.Value);
            return;
        }

        throw new Exception("Expected register or number node");
    }

    public override string ToString()
    {
        return "PUSH";
    }

    public override int GetSize()
    {
        return 1 + (Children.Count > 0 ? 1 : 0);
    }
}