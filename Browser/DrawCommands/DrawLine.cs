using Browser.Utils;
using SkiaSharp;

namespace Browser.DrawCommands;

public sealed class DrawLine : DrawCommand
{
    private readonly string _color;
    private readonly float _thickness;

    public DrawLine(float left,
        float top,
        float right,
        float bottom,
        string color,
        float thickness)
    {
        Rectangle = new SKRect(left, top, right, bottom);
        _color = color;
        _thickness = thickness;
    }

    public override void Draw(SKCanvas canvas)
    {
        var path = new SKPath();
        path.MoveTo(Rectangle.Left, Rectangle.Top);
        path.LineTo(Rectangle.Right, Rectangle.Bottom);
        var paint = new SKPaint
        {
            Color = ColorUtils.ParseColor(_color),
            StrokeWidth = _thickness,
            Style = SKPaintStyle.Stroke
        };
        canvas.DrawPath(path, paint);
    }
}