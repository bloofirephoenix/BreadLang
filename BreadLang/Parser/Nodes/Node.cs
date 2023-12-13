using BreadLang.Tokens;

namespace BreadLang.Parser.Nodes;

public abstract class Node
{
    public List<Node> Children { get; private set; } = new();

    public abstract void Populate(Parser parser);
    public abstract byte[] Compile();
    public abstract override string ToString();

    protected void PopulateAndAdd(Node node, Parser parser)
    {
        node.Populate(parser);
        Children.Add(node);
    }
}