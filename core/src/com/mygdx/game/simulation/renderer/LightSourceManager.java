package com.mygdx.game.simulation.renderer;

import org.joml.Vector3d;
import org.joml.Vector3f;

import java.util.ArrayList;
import java.util.Queue;
import java.util.concurrent.LinkedBlockingQueue;

/**
 * Created by Fran on 3/18/2018.
 */
public class LightSourceManager {
    private ArrayList<Shader> shaders;
    private final Vector3f temp;
    private Queue<LightEmitter> lightSources;
    private Camera camera;
    private Transformation transformation;

    public LightSourceManager(Camera camera, Transformation transformation)  {
        lightSources = new LinkedBlockingQueue<LightEmitter>();
        shaders = new ArrayList<Shader>();

        temp = new Vector3f();
        this.camera = camera;
        this.transformation = transformation;

    }

    public void addLight(LightEmitter light){
        lightSources.add(light);
    }

    public void removeLight(LightEmitter light){
        lightSources.remove(light);
    }

    public void update(){

        for(Shader shader : this.shaders) {
            int i = 0;
            shader.use();
            for (LightEmitter light : lightSources) {
                String s = "pointLights[" + i + "]";
                shader.setVec3f(s + ".position", light.getLightPosition(camera, transformation));
                shader.setVec3f(s + ".ambient", light.getAmbient());
                shader.setVec3f(s + ".diffuse", light.getDiffuse());
                shader.setVec3f(s + ".specular", light.getSpecular());

                i++;
                if (i == 8) {
                    break;
                }
            }

            shader.setInt("nLights", i);
        }
    }

    public Vector3d getTopAbsPos(){
        if(lightSources.isEmpty()){
            return new Vector3d(0,0,0);
        }
        return lightSources.peek().getAbsolutePosition();
    }

    public void addShader(Shader shader){
        shaders.add(shader);
    }

}
