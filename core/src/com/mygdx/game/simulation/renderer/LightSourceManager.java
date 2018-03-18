package com.mygdx.game.simulation.renderer;

import org.joml.Vector3d;
import org.joml.Vector3f;

import java.util.Queue;
import java.util.concurrent.ArrayBlockingQueue;
import java.util.concurrent.LinkedBlockingQueue;

/**
 * Created by Fran on 3/18/2018.
 */
public class LightSourceManager {
    private Shader shader;
    private Vector3f temp;
    private Queue<LightEmitter> lightSources;

    public LightSourceManager(Shader shader) {
        lightSources = new LinkedBlockingQueue<LightEmitter>();
        this.shader = shader;
        temp = new Vector3f();
    }

    public void addLight(LightEmitter light){
        lightSources.add(light);
    }

    public void removeLight(LightEmitter light){
        lightSources.remove(light);
    }

    public void update(){
        int i = 0;
        for(LightEmitter light : lightSources){
            String s = "pointLights[" + i + "]";
            shader.setVec3f(".position", light.getPosition());
            shader.setVec3f(".ambient", light.getAmbient());
            shader.setVec3f(".diffuse", light.getDiffuse());
            shader.setVec3f(".specular", light.getSpecular());

            i++;
            if(i==8){
                break;
            }
        }

        shader.setInt("nLights", i);

    }

}
