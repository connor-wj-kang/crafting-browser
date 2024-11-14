using Browser.Html;

namespace Browser.Css;

public sealed class DescendantSelector : Selector
{
    public readonly Selector Ancestor;
    public readonly Selector Descendant;

    public DescendantSelector(Selector ancestor, Selector descendant)
    {
        Ancestor = ancestor;
        Descendant = descendant;
        Priority = ancestor.Priority + descendant.Priority;
    }

    public override bool Matches(HtmlNode node)
    {
        if (!Descendant.Matches(node)) return false;
        while (node.Parent != null)
        {
            if (Ancestor.Matches(node.Parent)) return true;
            node = node.Parent;
        }

        return false;
    }
}