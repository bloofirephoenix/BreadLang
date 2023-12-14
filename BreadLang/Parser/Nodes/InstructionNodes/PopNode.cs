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
        throw new NotImplementedException();
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