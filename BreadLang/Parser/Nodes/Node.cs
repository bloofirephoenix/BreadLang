using BreadLang.Compiling;
using BreadLang.Tokens;

namespace BreadLang.Parser.Nodes;

public abstract class Node
{
    public List<Node> Children { get; private set; } = new();

    public abstract void Populate(Parser parser);
    public abstract void Compile(Compiler compiler);
    public abstract override string ToString();

    public abstract int GetSize();

    protected void PopulateAndAdd(Node node, Parser parser)
    {
        node.Populate(parser);
        Children.Add(node);
    }
}