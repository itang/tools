namespace Ext/Browser {
  type alias JDesktop = ##java.awt.Desktop
  type alias JURI = ##java.net.URI

  pub def open(url: String): Unit & Impure =
    import static java.awt.Desktop.getDesktop(): JDesktop & Impure ;
    import java.awt.Desktop.browse(JURI): Unit & Impure;
    import new java.net.URI(String): JURI & Impure as newURI;

    browse(getDesktop(), newURI(url))
}
