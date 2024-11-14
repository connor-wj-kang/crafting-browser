namespace Browser.Html;

public class HtmlText : HtmlNode
{
    public readonly string Text;

    public HtmlText(string text, HtmlNode? parent = null)
    {
        Text = text;
        Parent = parent;
    }
}