use model::ModelIterator;
use std::io;

pub struct ModelFormatter<S, W> {
    source: S,
    writer: W,
    seq_num: i64,
}

impl<S, W> ModelFormatter<S, W>
    where S: ModelIterator,
          W: io::Write,
{
    pub fn new(source: S, writer: W) -> ModelFormatter<S, W> {
        ModelFormatter {
            source,
            writer,
            seq_num: 0,
        }
    }

    // Our records are 49 bytes, with a 50th byte for a newline.
    pub fn write(&mut self) -> io::Result<()> {
        for model in &mut self.source {
            // h for header
            // seq_num
            // header id (padded to 5)
            // header name (padded to 20)
            write!(
                self.writer, 
                "h{:04}{:05}{:>20}                   \n",
                self.seq_num,
                model.header.id,
                model.header.name,
            )?;
            self.seq_num += 1;

            for location in &model.locations {
                // l for location
                // seq_num
                // address (padded to 20)
                write!(
                    self.writer,
                    "l{:04}{:>20}                        \n",
                    self.seq_num,
                    location.address,
                )?;
                self.seq_num += 1;

                for tx in &location.transactions {
                    // t for transaction
                    // seq_num
                    // amount (padded to 12)
                    // method (padded to 6)
                    write!(
                        self.writer,
                        "t{:04}{:012}{:>6}                          \n",
                        self.seq_num,
                        tx.amount,
                        tx.method,
                    )?;
                    self.seq_num += 1;
                }
            }
        }

        Ok(())
    }
}
