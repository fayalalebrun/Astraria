package com.company.algorithm.helpers;/*
* =====================================================================
* ==      Created by davrockenzahn19        ==    Date: 2/7/18   ==
* =====================================================================
* ==      Project: Generator tool    ==
* =====================================================================

*/

import java.io.IOException;
import java.io.ObjectOutputStream;
import java.io.OutputStream;

public class AppendableObjectOutputStream extends ObjectOutputStream {

    public AppendableObjectOutputStream(OutputStream out) throws IOException {
        super(out);
    }
    @Override
    protected void writeStreamHeader() throws IOException {
        // do not write a header
        reset();
    }
}


