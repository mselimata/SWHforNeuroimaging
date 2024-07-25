package com.example;

import java.io.IOException;

import org.softwareheritage.graph.SwhGraphProperties;
import org.softwareheritage.graph.SwhType;
import org.softwareheritage.graph.SwhUnidirectionalGraph;

import it.unimi.dsi.big.webgraph.LazyLongIterator;
import it.unimi.dsi.big.webgraph.labelling.ArcLabelledNodeIterator;



/**
 * Print the first 10 nodes of a given type [snapshot=snp] in a given graph.
 * And count the number of nodes of this type.
 * /home/matay/datasets/python3k/graph
 * /home/matay/swh-environment/swh-graph/swh/graph/example_dataset/compressed/example
 */
 
public class App 
{
    static final String DEFAULT_GRAPH_PATH = "/home/matay/datasets/python3k/graph";

    static final int PRINT_NODE_LIMIT = 10;

    static final SwhType NODE_TYPE_ORIGIN = SwhType.ORI;

    static final SwhType NODE_TYPE_SNAPSHOT = SwhType.SNP;

    static final SwhType NODE_TYPE_DIRECTORY = SwhType.DIR;
    

    public static void main( String[] args ) throws IOException
    {
        // load the labelled graph in memory
        SwhUnidirectionalGraph graph = SwhUnidirectionalGraph.loadLabelled(DEFAULT_GRAPH_PATH);

        System.out.println("there are " + graph.numNodes() + " nodes.");

        // iterate over the nodes
        ArcLabelledNodeIterator iterator = graph.labelledNodeIterator();
        
        int nb_nodes_with_type = 0;
        long startTime = System.nanoTime();
        while (iterator.hasNext()) {
            long node_id = iterator.nextLong();
            SwhType node_type = graph.getNodeType(node_id); // get the type of the node

            if (node_type == NODE_TYPE_ORIGIN) {
                if (nb_nodes_with_type < PRINT_NODE_LIMIT) {
                    System.out.println("node id: " + node_id + ", node SWHID : " + graph.getSWHID(node_id));
                }
                nb_nodes_with_type++;
            
            }
        }
        long endTime = System.nanoTime();
        long duration = (endTime - startTime);
        float duration_in_seconds = (float)duration / 1_000_000_000;

        System.out.println("there are " + nb_nodes_with_type + " nodes of type " + NODE_TYPE_TO_FILTER);
        System.out.println("it took " + duration_in_seconds + " seconds to iterate over the nodes.");
    }
}
