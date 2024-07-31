package com.example;

import java.io.IOException;

import org.softwareheritage.graph.SWHID;
import org.softwareheritage.graph.SwhGraphProperties;
import org.softwareheritage.graph.SwhType;
import org.softwareheritage.graph.SwhUnidirectionalGraph;
import org.softwareheritage.graph.labels.DirEntry;

import it.unimi.dsi.big.webgraph.LazyLongIterator;
import it.unimi.dsi.big.webgraph.labelling.ArcLabelledNodeIterator;

public class Appcp {

    public static void printEntries(SwhUnidirectionalGraph g, long dirNode) {
        ArcLabelledNodeIterator.LabelledArcIterator s = g.labelledSuccessors(dirNode);
        for (long dst; (dst = s.nextLong()) >= 0;) {
            DirEntry[] labels = (DirEntry[]) s.label().get();
            for (DirEntry label : labels) {
                System.out.format(
                    "%s %s %d\n",
                    g.getSWHID(dst),
                    new String(g.getLabelName(label.filenameId)),
                    label.permission
                );
            }
        }
    }

    // Usage: $PROGRAM <GRAPH_BASENAME> <DIR_SWHID>
    public static void main(String[] args) throws IOException {
        SwhUnidirectionalGraph g = SwhUnidirectionalGraph.loadLabelledMapped(args[0]);
        g.loadLabelNames();
        long dirNode = g.getNodeId(new SWHID(args[1]));
        printEntries(g, dirNode);
    }
}