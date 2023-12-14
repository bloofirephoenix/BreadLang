using BreadLang.Compiling;
using BreadLang.Tokens;

namespace BreadLang.Parser.Nodes;

public class ProgramNode : Node
{
    private string _indent = string.Empty;
    public bool CheckIndent(Token token)
    {
        if (token.Type != TokenType.Indent)
        {
            return false;
        }

        if (_indent == string.Empty)
        {
            _indent = token.Lexeme;
            return true;
        }

        return _indent == token.Lexeme;
    }

    public override void Populate(Parser parser)
    {
        while (!parser.IsAtEnd())
        {
            parser.SkipNewLines();

            if (parser.Peek().Type == TokenType.Eof)
                return;

            var identifier = parser.Expect(TokenType.Identifier);
            parser.Expect(TokenType.Colon);

            var name = identifier.Lexeme;

            var sub = new SubRoutineNode(this, name);
            sub.Populate(parser);
            Children.Add(sub);
        }
    }

    public override void Compile(Compiler compiler)
    {
        // first we need to order the subroutines. main is always first.
        List<SubRoutineNode> subroutines = new();
        foreach (var child in Children)
        {
            SubRoutineNode node = (SubRoutineNode)child;
            if (node.Name == "main")
                subroutines.Insert(0, node);
            else
                subroutines.Add(node);
        }

        // get the size of location subroutine to insert into the placeholders
        int currentLocation = 0;
        foreach (var sub in subroutines)
        {
            compiler.Placeholders.Add(sub.Name, compiler.GetImmediate16(currentLocation));
            int size = sub.GetSize();
            currentLocation += size;
        }

        // time to compile
        foreach (var sub in subroutines)
        {
            sub.Compile(compiler);
        }
    }

    public override string ToString()
    {
        return "Program";
    }

    public override int GetSize()
    {
        throw new NotImplementedException();
    }
}