package com.mygdx.game;

import com.badlogic.gdx.files.FileHandle;
import com.badlogic.gdx.graphics.Color;
import com.badlogic.gdx.graphics.Texture;
import com.badlogic.gdx.graphics.VertexAttributes;
import com.badlogic.gdx.graphics.g3d.Material;
import com.badlogic.gdx.graphics.g3d.Model;
import com.badlogic.gdx.graphics.g3d.attributes.ColorAttribute;
import com.badlogic.gdx.graphics.g3d.utils.ModelBuilder;
import com.mygdx.game.logic.DetailedBody;

import java.util.Vector;

/**
 * Created by fraayala19 on 12/18/17.
 */
public class SaveFileManager {
    public static void loadGame(SimulationScreen simulationScreen, FileHandle file){
        simulationScreen.removeBodies(simulationScreen.getBodies());

        String str = file.readString();
        String part[] = str.replaceAll("\\r", "").split("\\n");

        ModelBuilder modelBuilder = new ModelBuilder();

     if (part[0].contains("v2")){
            boolean useColor = false, useTexture = false;

            Vector<DetailedBody> planetList = new Vector<DetailedBody>();

            String name;
            double radius;
            double mass;
            double vX=0;
            double vY=0;
            double vZ=0;
            float r=0,g=0,b=0,a=0;
            double x;
            double y;
            double z;

            DetailedBody p = null;

            int i = 2;

            while(i<part.length-2){
                if(part[i].contains("name")){
                    name = part[i].substring(part[i].indexOf(':')+2);
                    i++;
                    radius = Double.parseDouble(part[i].substring(part[i].indexOf(':')+2));
                    i++;
                    radius *= 1000;
                    mass = Double.parseDouble(part[i].substring(part[i].indexOf(':')+2));
                    i++;
                    mass *= 5.976E24;
                    vX = Double.parseDouble(part[i].substring(part[i].indexOf(':')+2));
                    i++;
                    vY = Double.parseDouble(part[i].substring(part[i].indexOf(':')+2));
                    i++;
                    vZ = Double.parseDouble(part[i].substring(part[i].indexOf(':')+2));
                    i++;
                    if(part[i].contains("color")) {
                        useColor = true;
                        r = Float.parseFloat(part[i].substring(part[i].indexOf(':') + 2));
                        i++;
                        g = Float.parseFloat(part[i].substring(part[i].indexOf(':') + 2));
                        i++;
                        b = Float.parseFloat(part[i].substring(part[i].indexOf(':') + 2));
                        i++;
                        a = Float.parseFloat(part[i].substring(part[i].indexOf(':') + 2));
                        i++;
                    } else {
                        useTexture = true;
                        i++;
                        r = Float.parseFloat(part[i].substring(part[i].indexOf(':') + 2));
                        i++;
                        g = Float.parseFloat(part[i].substring(part[i].indexOf(':') + 2));
                        i++;
                        b = Float.parseFloat(part[i].substring(part[i].indexOf(':') + 2));
                        i++;
                        a = Float.parseFloat(part[i].substring(part[i].indexOf(':') + 2));
                        i++;
                    }
                    x = Double.parseDouble(part[i].substring(part[i].indexOf(':')+2));
                    i++;
                    y = Double.parseDouble(part[i].substring(part[i].indexOf(':')+2));
                    i++;
                    z = Double.parseDouble(part[i].substring(part[i].indexOf(':')+2));
                    i++;
                    Color c = new Color(r, g, b, a);

                    final Material material = new Material(ColorAttribute.createDiffuse(c));
                    final long attributes = VertexAttributes.Usage.Position | VertexAttributes.Usage.Normal;
                    Model model = modelBuilder.createSphere(1, 1, 1, 24, 24, material, attributes);

                    planetList.add(new DetailedBody(mass, x, y, z, vX, vY, vZ, model, radius, name));

                    useColor = false;
                    useTexture = false;
                } else {
                    i++;
                }
               // System.out.println(vX+""+vY+" "+vZ);
            }
            simulationScreen.addBodies(planetList);


        }
    }
}
