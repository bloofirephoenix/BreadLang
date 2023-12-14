using BreadLang.Compiling;
using BreadLang.Tokens;

namespace BreadLang.Parser.Nodes.InstructionNodes;

public class SubNode : Node
{
    public override void Populate(Parser parser)
    {
        PopulateAndAdd(new RegisterNode(), parser);

        if (parser.Check(TokenType.Register))
            PopulateAndAdd(new RegisterNode(), parser);
        else if (parser.Check(TokenType.Number))
            PopulateAndAdd(new NumberNode(NumberNode.Type.Immediate8), parser);
        else
            ErrorHandler.Instance!.Error(parser.Current(), "Expected register or number");
    }

    public override void Compile(Compiler compiler)
    {
        RegisterSelect register1 = compiler.GetRegister(((RegisterNode)Children[0]).Register);
        Node registerOrNumber = Children[1];

        if (registerOrNumber is NumberNode numNode)
        {
            compiler.WriteFirstByte(OpCodes.Sub, true, register1);
            compiler.Write((byte)numNode.Value);
            return;
        }

        if (registerOrNumber is RegisterNode regNode)
        {
            compiler.WriteTwoBytes(OpCodes.Sub, true, register1, compiler.GetRegister(regNode.Register));
            return;
        }

        throw new Exception("Expected register or number node");
    }

    public override string ToString()
    {
        return "SUB";
    }

    public override int GetSize()
    {
        return 2;
    }
}