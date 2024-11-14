namespace Browser.Html;

public abstract class HtmlNode
{
    public readonly List<HtmlNode> Children = [];
    public readonly Dictionary<string, string> Styles = new();
    public bool IsFocused = false;
    public HtmlNode? Parent = null;
}