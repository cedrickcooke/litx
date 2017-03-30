use std::io;

pub trait Graphviz<W: io::Write> {
    fn write_graph(&self, writer: &mut W) -> io::Result<()> {
        writeln!(writer, "strict digraph {{")?;
        self.write_edges(writer)?;
        writeln!(writer, "}}")?;
        Ok(())
    }

    fn get_vertex_name(&self) -> String;

    #[allow(unused_variables)]
    fn write_edges(&self, writer: &mut W) -> io::Result<()> { Ok(()) }
}
