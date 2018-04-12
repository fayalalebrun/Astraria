package com.mygdx.game.logic.algorithms;

import com.mygdx.game.logic.Body;
import com.mygdx.game.logic.helpers.Units;

/**
 * Created by fraayala19 on 4/12/18.
 */
public class VelocityVerlet extends NBodyAlgorithm{
    private double temporary, ax, ay, az;

    public VelocityVerlet() {
        super();
    }

    @Override
    protected void runAlgorithm() {
            double delta = getDelta()*1;
            //delta *= GameScreen.simSpeed;
        synchronized (bodies) {
            for (Body b : this.bodies) {

                ax = 0;
                ay = 0;
                az = 0;

                double pX = b.getX();
                double pY = b.getY();
                double pZ = b.getZ();

                if (!b.getAccelInit()) {
                    for (int i = 0; i < this.bodies.indexOf(b); i++) {
                        parseAcceleration(bodies.get(i), pX, pY, pZ);
                    }
                    for (int i = this.bodies.indexOf(b) + 1; i < this.bodies.size(); i++) {
                        parseAcceleration(bodies.get(i), pX, pY, pZ);
                    }

                    ax *= Units.GRAV;
                    ay *= Units.GRAV;
                    az *= Units.GRAV;

                    b.setCurrAccelX(ax);
                    b.setCurrAccelY(ay);
                    b.setCurrAccelZ(az);

                    b.setAccelInitTrue();
                }

                synchronized (b) {
                    pX = pX + (b.getvX() * delta) + (0.5 * b.getCurrAccelX() * sq(delta));
                    pY = pY + (b.getvY() * delta) + (0.5 * b.getCurrAccelY() * sq(delta));
                    pZ = pZ + (b.getvY() * delta) + (0.5 * b.getCurrAccelZ() * sq(delta));

                    b.setX(pX);
                    b.setY(pY);
                    b.setZ(pZ);
                }

                ax = 0;
                ay = 0;
                az = 0;

                for (int i = 0; i < this.bodies.indexOf(b); i++) {
                    parseAcceleration(bodies.get(i), pX, pY, pZ);
                }
                for (int i = this.bodies.indexOf(b) + 1; i < this.bodies.size(); i++) {
                    parseAcceleration(bodies.get(i), pX, pY, pZ);
                }
                ax *= Units.GRAV;
                ay *= Units.GRAV;
                az *= Units.GRAV;

                b.setvX(b.getvX() + (((b.getCurrAccelX() + ax) / 2) * delta));
                b.setvY(b.getvY() + (((b.getCurrAccelY() + ay) / 2) * delta));
                b.setvZ(b.getvZ() + (((b.getCurrAccelZ() + az) / 2) * delta));

                b.setCurrAccelX(ax);
                b.setCurrAccelY(ay);
                b.setCurrAccelZ(az);
            }
        }
    }

    private void parseAcceleration(Body other, double pX, double pY, double pZ){
        double oX = other.getX();
        double oY = other.getY();
        double oZ = other.getY();
        temporary = other.getMass() / cb(Math.sqrt(sq(oX-pX) + sq(oY-pY) + sq(oZ - pZ)));
        ax += (oX - pX) * temporary;
        ay += (oY - pY) * temporary;
        az += (oZ - pZ) * temporary;
    }
}
