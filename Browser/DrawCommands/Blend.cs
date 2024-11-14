using Browser.Html;
using SkiaSharp;

namespace Browser.DrawCommands;

public sealed class Blend : DrawCommand
{
    private readonly string _blendMode;
    private readonly List<DrawCommand> _children;
    private readonly float _opacity;
    private readonly bool _shouldSave;

    public Blend(float opacity, string blendMode, List<DrawCommand> children)
    {
        _opacity = opacity;
        _blendMode = blendMode;
        _shouldSave = blendMode != "" || _opacity < 1;
        _children = children;
        Rectangle = SKRect.Empty;
        _children.ForEach(child => Rectangle.Union(child.Rectangle));
    }

    public override void Draw(SKCanvas canvas)
    {
        var paint = new SKPaint
        {
            MaskFilter = SKMaskFilter.CreateBlur(SKBlurStyle.Normal, _opacity),
            BlendMode = ParseBlendMode(_blendMode)
        };
        if (_shouldSave) canvas.SaveLayer(paint);
        _children.ForEach(child => child.Draw(canvas));
        if (_shouldSave) canvas.Restore();
    }

    public static SKBlendMode ParseBlendMode(string blendMode)
    {
        return blendMode switch
        {
            "multiply" => SKBlendMode.Multiply,
            "difference" => SKBlendMode.Difference,
            "destination-in" => SKBlendMode.DstIn,
            "source-over" => SKBlendMode.SrcOver,
            _ => SKBlendMode.SrcOver
        };
    }

    public static List<DrawCommand> PaintVisualEffects(HtmlNode node,
        List<DrawCommand> drawCommands, SKRect rectangle)
    {
        var opacity =
            (float)Convert.ToDouble(
                node.Styles.GetValueOrDefault("opacity", "1.0"));
        var blendMode = node.Styles.GetValueOrDefault("mix-blend-mode", "");
        if (node.Styles.GetValueOrDefault("overflow", "visible") != "clip")
            return [new Blend(opacity, blendMode, drawCommands)];
        var borderRadius =
            (float)Convert.ToDouble(
                node.Styles.GetValueOrDefault("border-radius",
                    "0px")[..^2]);
        if (blendMode == "") blendMode = "source-over";
        drawCommands.Add(new Blend(1.0f, "destination-in",
            [new DrawRoundRectangle(rectangle, borderRadius, "white")]));
        return [new Blend(opacity, blendMode, drawCommands)];
    }
}