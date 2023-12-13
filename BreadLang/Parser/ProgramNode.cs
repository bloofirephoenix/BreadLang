using BreadLang.Parser.Nodes;
using BreadLang.Tokens;

namespace BreadLang.Parser;

public class ProgramNode : Node
{
    public Dictionary<string, uint> SubRoutineLocations = new();

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

    public override byte[] Compile()
    {
        throw new NotImplementedException();
    }

    public override string ToString()
    {
        return "Program";
    }
}