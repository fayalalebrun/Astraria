package com.mygdx.game.simulation.renderer;

import org.joml.Vector3f;

import java.util.Queue;
import java.util.concurrent.LinkedBlockingQueue;

/**
 * Created by Fran on 3/18/2018.
 */
public class LightSourceManager {
    private Shader shader;
    private final Vector3f temp;
    private Queue<LightEmitter> lightSources;
    private Camera camera;
    private Transformation transformation;

    public LightSourceManager(Shader shader, Camera camera, Transformation transformation)  {
        lightSources = new LinkedBlockingQueue<LightEmitter>();
        this.shader = shader;
        temp = new Vector3f();
        this.camera = camera;
        this.transformation = transformation;

        try {
            shader.use();
            for (int i = 0; i < 8; i++) {
                String s = "pointLights[" + i + "]";
                shader.createUniform(s + ".position");
                shader.createUniform(s+".ambient");
                shader.createUniform(s+".diffuse");
                shader.createUniform(s+".specular");
            }
            shader.createUniform("nLights");
            shader.setInt("nLights",0);
        } catch (Exception e){
            e.printStackTrace();
        }
    }

    public void addLight(LightEmitter light){
        lightSources.add(light);
    }

    public void removeLight(LightEmitter light){
        lightSources.remove(light);
    }

    public void update(){
        int i = 0;
        shader.use();
        for(LightEmitter light : lightSources){
            String s = "pointLights[" + i + "]";
            shader.setVec3f(s+".position", light.getLightPosition(camera, transformation));
            shader.setVec3f(s+".ambient", light.getAmbient());
            shader.setVec3f(s+".diffuse", light.getDiffuse());
            shader.setVec3f(s+".specular", light.getSpecular());

            i++;
            if(i==8){
                break;
            }
        }

        shader.setInt("nLights", i);

    }

}
