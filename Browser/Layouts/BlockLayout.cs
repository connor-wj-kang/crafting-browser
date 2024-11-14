using Browser.Html;
using Browser.Utils;

namespace Browser.Layouts;

public sealed class BlockLayout : Layout
{
    public readonly float CursorX;

    public void Word(HtmlNode node, string word)
    {
        var weight = node.Styles["font-weight"];
        var style = node.Styles["font-style"];
        var size = (float)(Convert.ToDouble(node.Styles["font-size"][..^2]) *
                           0.75);
        var font = FontUtils.GetFont(size, weight, style);
        var width = font.MeasureText(word);
        if (CursorX + width > Width) NewLine();
        var line = Children.Last();
        var previousWord = line.Children.LastOrDefault();
        var text = new TextLayout(node, word, line, previousWord);
    }

    public void NewLine()
    {
    }
}