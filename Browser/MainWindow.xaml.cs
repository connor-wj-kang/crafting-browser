using System.Windows;
using Browser.Css;
using Browser.DrawCommands;
using Browser.Html;
using Browser.Layouts;
using SkiaSharp;
using SkiaSharp.Views.Desktop;

namespace Browser;

/// <summary>
///     Interaction logic for MainWindow.xaml
/// </summary>
public partial class MainWindow : Window
{
    public MainWindow()
    {
        InitializeComponent();
    }

    private void OnPaintSurface(object sender, SKPaintSurfaceEventArgs e)
    {
        // the the canvas and properties
        var canvas = e.Surface.Canvas;
        var rules = new CssParser("").Parse();
        // make sure the canvas is blank
        canvas.Clear(SKColors.White);

        var nodes = new HtmlParser("hello").Parse();
        CssParser.ApplyCss(nodes, rules);
        var document = new DocumentLayout(nodes);
        document.CalculateLayout();
        var displayList = new List<DrawCommand>();
        Layout.PaintTree(document, displayList);
        displayList.ForEach(cmd => cmd.Draw(canvas));

        // draw some text
    }
}