using BreadLang.Compiling;
using BreadLang.Parser.Nodes;

namespace BreadLang;

public class MacroNode : Node
{
    public override void Compile(Compiler compiler)
    {
        throw new NotImplementedException();
    }

    public override int GetSize()
    {
        throw new NotImplementedException();
    }

    public override void Populate(Parser.Parser parser)
    {
        throw new NotImplementedException();
    }

    public override string ToString()
    {
        throw new NotImplementedException();
    }
}
