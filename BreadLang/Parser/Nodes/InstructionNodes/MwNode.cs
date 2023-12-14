using BreadLang.Compiling;
using BreadLang.Tokens;

namespace BreadLang.Parser.Nodes.InstructionNodes;

public class MwNode : Node
{
    public override void Populate(Parser parser)
    {
        PopulateAndAdd(new RegisterNode(), parser);

        parser.Expect(TokenType.Comma);

        if (parser.Check(TokenType.Register))
            PopulateAndAdd(new RegisterNode(), parser);
        else if (parser.Check(TokenType.Number))
            PopulateAndAdd(new NumberNode(NumberNode.Type.Immediate8), parser);
        else
            ErrorHandler.Instance!.Error(parser.Current(), "Expected register or number");
    }

    public override void Compile(Compiler compiler)
    {
        RegisterSelect reg1 = compiler.GetRegister(((RegisterNode)Children[0]).Register);
        Node second = Children[1];

        if (second is NumberNode num)
        {
            compiler.WriteFirstByte(OpCodes.Mw, true, reg1);
            compiler.Write((byte) num.Value);
            return;
        }

        if (second is RegisterNode reg2)
        {
            compiler.WriteTwoBytes(OpCodes.Mw, false, reg1, compiler.GetRegister(reg2.Register));
            return;
        }

        throw new Exception("Expected number or register");
    }

    public override string ToString()
    {
        return "Mw";
    }

    public override int GetSize()
    {
        return 2;
    }
}