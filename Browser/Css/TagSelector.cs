using Browser.Html;

namespace Browser.Css;

public sealed class TagSelector : Selector
{
    public readonly string TagName;

    public TagSelector(string tagName)
    {
        TagName = tagName;
        Priority = 1;
    }

    public override bool Matches(HtmlNode node)
    {
        return node is HtmlElement htmlElement &&
               htmlElement.TagName == TagName;
    }
}