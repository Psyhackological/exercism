""" RNA Transcription module """


def to_rna(dna_strand):
    """To RNA function

    Arguments:
        dna_strand (str): DNA strand

    Returns:
        str: RNA complement
    """

    dna_strand = dna_strand.strip().upper()

    rna_complement = (
        dna_strand.replace("A", "U")
        .replace("T", "A")
        .replace("G", "TEMP")
        .replace("C", "G")
        .replace("TEMP", "C")
    )

    return rna_complement
