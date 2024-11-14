using SkiaSharp;

namespace Browser.Utils;

public static class ColorUtils
{
    private static readonly Dictionary<string, string> NamedColors = new()
    {
        { "black", "#000000" },
        { "gray", "#808080" },
        { "white", "#ffffff" },
        { "red", "#ff0000" },
        { "green", "#00ff00" },
        { "blue", "#0000ff" },
        { "lightblue", "#add8e6" },
        { "lightgreen", "#90ee90" },
        { "orange", "#ffa500" },
        { "orangered", "#ff4500" }
    };

    public static SKColor ParseColor(string color)
    {
        if (color.StartsWith('#') && color.Length == 7)
        {
            var (r, g, b) = (Convert.ToByte(color[1..3], 16),
                Convert.ToByte(color[3..5], 16),
                Convert.ToByte(color[5..7], 16));
            return new SKColor(r, g, b);
        }

        if (color.StartsWith('#') && color.Length == 9)
        {
            var (r, g, b, a) = (Convert.ToByte(color[1..3], 16),
                Convert.ToByte(color[3..5], 16),
                Convert.ToByte(color[5..7], 16),
                Convert.ToByte(color[7..9], 16));
            return new SKColor(r, g, b, a);
        }

        if (NamedColors.TryGetValue(color, out var value))
            return ParseColor(value);
        return SKColors.Black;
    }
}