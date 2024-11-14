using Browser.Utils;
using SkiaSharp;

namespace Browser.DrawCommands;

public sealed class DrawText
    : DrawCommand
{
    private readonly string _color;
    private readonly SKFont _font;
    private readonly string _text;

    public DrawText(
        float left,
        float top,
        string text,
        SKFont font,
        string color)
    {
        Rectangle = new SKRect
        {
            Left = left,
            Top = top,
            Right = left + font.MeasureText(text),
            Bottom = top - font.Metrics.Ascent + font.Metrics.Descent
        };
        _font = font;
        _color = color;
        _text = text;
    }

    public override void Draw(SKCanvas canvas)
    {
        var paint = new SKPaint
        {
            IsAntialias = true,
            Color = ColorUtils.ParseColor(_color)
        };
        var baseline = Rectangle.Top - _font.Metrics.Ascent;
        canvas.DrawText(_text, Rectangle.Left, baseline, _font, paint);
    }
}