""" RNA Transcription module """


def to_rna(dna_strand):
    """To RNA function

    Arguments:
        dna_strand (str): DNA strand

    Returns:
        str: RNA complement
    """

    dna_strand = dna_strand.strip().upper()

    from_chars = "GCTA"
    to_chars = "CGAU"

    translate_table = dna_strand.maketrans(from_chars, to_chars)

    rna_complement = dna_strand.translate(translate_table)

    return rna_complement
