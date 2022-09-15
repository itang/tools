namespace Ext/Browser {
  type alias JDesktop = ##java.awt.Desktop
  type alias JURI = ##java.net.URI

  pub def open(url: String): Unit \ IO  =
    import static java.awt.Desktop.getDesktop(): JDesktop \ IO;
    import java.awt.Desktop.browse(JURI): Unit \ IO;
    import new java.net.URI(String): JURI \ IO as newURI;

    browse(getDesktop(), newURI(url))
}
