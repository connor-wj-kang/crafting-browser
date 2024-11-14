using Browser.Html;

namespace Browser.Css;

public abstract class Selector
{
    public int Priority;
    public abstract bool Matches(HtmlNode node);
}