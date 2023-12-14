using BreadLang.Compiling;
using BreadLang.Tokens;

namespace BreadLang.Parser.Nodes.InstructionNodes;

public class JmpNode : Node
{
    public override void Populate(Parser parser)
    {
        if (parser.Check(TokenType.Identifier))
        {
            PopulateAndAdd(new PlaceholderNode(), parser);
        }
    }

    public override void Compile(Compiler compiler)
    {
        if (Children.Count == 0)
        {
            compiler.WriteFirstByte(OpCodes.Jmp, false, null);
        }
        else
        {
            compiler.WriteFirstByte(OpCodes.Jmp, true, null);
            Children[0].Compile(compiler);
        }
    }

    public override string ToString()
    {
        return "JMP";
    }

    public override int GetSize()
    {
        if (Children.Count > 0) // if immediate value is used
            return 3;

        return 1;
    }
}