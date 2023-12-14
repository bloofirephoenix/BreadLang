using BreadLang.Compiling;

namespace BreadLang.Parser.Nodes.InstructionNodes;

public class PopNode : Node
{
    public override void Populate(Parser parser)
    {
        var reg = new RegisterNode();
        reg.Populate(parser);
        Children.Add(reg);
    }

    public override void Compile(Compiler compiler)
    {
        var reg = (RegisterNode) Children[0];
        compiler.WriteFirstByte(OpCodes.Pop, false, compiler.GetRegister(reg.Register));
    }

    public override string ToString()
    {
        return "POP";
    }

    public override int GetSize()
    {
        return 1;
    }
}