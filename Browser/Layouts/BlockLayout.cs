using Browser.DrawCommands;
using Browser.Html;
using Browser.Utils;
using SkiaSharp;

namespace Browser.Layouts;

public enum LayoutMode
{
    Inline,
    Block
}

public sealed class BlockLayout(
    HtmlNode node,
    Layout? parent = null,
    Layout? previous = null)
    : Layout(node, parent, previous)
{
    private float _cursorX;

    private static readonly string[] BlockElements =
    [
        "html", "body", "article", "section", "nav", "aside",
        "h1", "h2", "h3", "h4", "h5", "h6", "hgroup", "header",
        "footer", "address", "p", "hr", "pre", "blockquote",
        "ol", "ul", "menu", "li", "dl", "dt", "dd", "figure",
        "figcaption", "main", "div", "table", "form", "fieldset",
        "legend", "details", "summary"
    ];

    private void Recurse(HtmlNode node)
    {
        switch (node)
        {
            case HtmlText htmlText:
            {
                foreach (var word in htmlText.Text.Split(' '))
                {
                    Word(node, word); 
                }
                break;
            }
            case HtmlElement { TagName: "br" }:
                NewLine();
                break;
            case HtmlElement { TagName: "input" } or HtmlElement { TagName: "button" }:
                Input(node);
                break;
            default:
            {
                node.Children.ForEach(Recurse);
                break;
            }
        }
    }

    private LayoutMode GetLayoutMode()
    {
        if (Node is HtmlText)
        {
            return LayoutMode.Inline;
        }
        else if (Node.Children.Exists(child =>
                     child is HtmlElement htmlElement &&
                    BlockElements.Contains(htmlElement.TagName)))
        {
            return LayoutMode.Block;
        } else if (Node.Children.Count != 0 ||
                   (Node is HtmlElement { TagName: "input" }))
        {
            return LayoutMode.Inline;
        }
        else
        {
            return LayoutMode.Block;
        }
        
    }

    private void Word(HtmlNode node, string word)
    {
        var weight = node.Styles["font-weight"];
        var style = node.Styles["font-style"];
        var size = FontUtils.ParseFontSize(node.Styles["font-size"][..^2]);
        var font = FontUtils.GetFont(size, weight, style);
        var width = font.MeasureText(word);
        if (_cursorX + width > Width) NewLine();
        var line = Children.Last();
        var previousWord = line.Children.LastOrDefault();
        var text = new TextLayout(node, word, line, previousWord);
        line.Children.Add(text);
        _cursorX += width + font.MeasureText(" ");
    }

    private void Input(HtmlNode node)
    {
        const float width = InputLayout.InputWidthPx;
        if (_cursorX + width > Width) NewLine();
        var line = Children.Last(); 
        var previousWord = line.Children.LastOrDefault();
        var input = new InputLayout(node, line, previousWord);
        line.Children.Add(input);
        var weight = node.Styles["font-weight"];
        var style = node.Styles["font-style"];
        var size = FontUtils.ParseFontSize(node.Styles["font-size"][..^2]);
        var font = FontUtils.GetFont(size, weight, style);
        _cursorX += width + font.MeasureText(" ");
    }

    private void NewLine()
    {
        _cursorX = 0;
        var lastLine = Children.LastOrDefault();
        var newLine = new LineLayout(Node, this, lastLine);
        Children.Add(newLine);
    }

    private SKRect GetBlockLayoutRectangle()
    {
        return new SKRect(X, Y, X + Width, Y + Height);
    }

    public override void CalculateLayout()
    {
        Width = Parent!.Width;
        X = Parent.X;
        if (Previous != null)
        {
            Y = Previous.Y + Previous.Height;
        }
        else
        {
            Y = Parent.Y;
        }
        var mode = GetLayoutMode();
        if (mode == LayoutMode.Block)
        {
            Layout? previous = null;
            Node.Children.ForEach(child =>
            {
                var next = new BlockLayout(child, this, previous);
                Children.Add(next);
                previous = next;
            });
        }
        else
        {
            NewLine();
            Recurse(Node);
        }
        Children.ForEach(child => child.CalculateLayout());
        Height = Children.Select(child => child.Height).Sum();
    }

    public override List<DrawCommand> Paint()
    {
        var drawCommands = new List<DrawCommand>();
        var bgColor = Node.Styles.GetValueOrDefault("background-color", "transparent");
        if (bgColor == "transparent") return drawCommands;
        var radius = (float)Convert.ToDouble(
            Node.Styles.GetValueOrDefault("border-radius", "0px")[0..^2]);
        drawCommands.Add(new DrawRoundRectangle(GetBlockLayoutRectangle(), radius, bgColor));
        return drawCommands;
    }

    public override bool ShouldPaint()
    {
        return true;
    }

    public override List<DrawCommand> PaintEffects(List<DrawCommand> drawCommands)
    {
        return Blend.PaintVisualEffects(Node, drawCommands,
            GetBlockLayoutRectangle());
    }
}