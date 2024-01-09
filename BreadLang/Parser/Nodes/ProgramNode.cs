using BreadLang.Compiling;
using BreadLang.Tokens;

namespace BreadLang.Parser.Nodes;

public class ProgramNode : Node
{
    public Dictionary<string, Macro> Macros = new();
    
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

            if (parser.Check(TokenType.Eof))
                return;

            if (parser.Check(TokenType.Macro))
                Macro(parser);
            else
                SubRoutine(parser);
        }
    }

    public void SubRoutine(Parser parser) 
    {
        var identifier = parser.Expect(TokenType.Identifier);

        parser.Expect(TokenType.Colon);

        var name = identifier.Lexeme;

        var sub = new SubRoutineNode(this, name);
        sub.Populate(parser);
        Children.Add(sub);
    }

    public void Macro(Parser parser)
    {
        parser.Advance(); // advance past @macro
        parser.Advance(); // advance past new line
        
        var name = parser.Expect(TokenType.Identifier).Lexeme;

        List<string> arguments = new();

        // arguments
        if (parser.Check(TokenType.OpenParenthesis))
        {
            _ = parser.Advance(); // advance past open parenthesis
            
            while (!parser.Check(TokenType.CloseParenthesis))
            {   
                // expect an identifier
                var arg = parser.Expect(TokenType.Identifier);

                arguments.Add(arg.Lexeme); // add argument name to the list.
                
                // expect either close parenthesis (break out of loop) or a comma (continue)
                if (parser.Check(TokenType.CloseParenthesis))
                    break;
                
                parser.Expect(TokenType.Comma);
            }
            if (parser.Check(TokenType.CloseParenthesis))
                parser.Advance();
            
            parser.Expect(TokenType.Colon);
        }

        var macro = new Macro(this, arguments);
        macro.ReadTokens(parser);

        Macros.Add(name, macro);
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