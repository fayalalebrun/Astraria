package com.mygdx.game.simulation;

import com.mygdx.game.simulation.renderer.*;
import sun.security.provider.SHA;

/**
 * Created by Fran on 4/10/2018.
 */
public class AtmospherePlanet extends SimulationObject{
    Sphere atmosphereSphere;
    Shader atmosphereShader;

    float atmosphericRadius;

    public AtmospherePlanet(double x, double y, double z, Model model, Shader shader, Shader atmosphereShader, float radius, String name, Renderer renderer) {
        super(x, y, z, model, shader, radius, name);

        this.atmosphereShader = atmosphereShader;

        atmosphereSphere = new Sphere(renderer.getModelManager(),renderer.getTransformation(),-1);
        
        atmosphericRadius = radius*1.025f;
    }

    @Override
    protected void update(Camera cam) {
        super.update(cam);

        atmosphereSphere.setScale(getRadius());
        atmosphereSphere.setPosition(getPositionRelativeToCamera(cam));
    }

    @Override
    public void render(Camera cam) {
        super.render(cam);

        atmosphereSphere.render(cam, atmosphereShader);
    }

    @Override
    public void dispose() {
        super.dispose();
        atmosphereSphere.dispose();
    }
}
