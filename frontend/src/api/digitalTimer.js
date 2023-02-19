import createAxiosInstance from './baseApi';

/**
 * @typedef {Object} DigitalTimer
 * @property {number} id
 * @property {string} name
 * @property {number} pin
 * @property {number} hour
 * @property {number} minute
 * @property {number} duration
 * @property {boolean} is_enabled
 */

const digitalTimerApi = {
    /**
     * Get a digital timer by its id.
     * @param {number} id 
     * @returns {Promise<DigitalTimer>}
     */
    async get(id) {
        const res = await createAxiosInstance().get(`api/digital_timer/${id}`);
        return res.data;
    },

    /**
     * List digital timers.
     * @returns {Promise<DigitalTimer[]>}
     */
    async list() {
        const res = await createAxiosInstance().get('api/digital_timer');
        return res.data;
    },

    /**
     * Create a new digital timer.
     * @param {DigitalTimer} timer Digital timer to create.
     * @returns {Promise<DigitalTimer>}
     */
    async create(timer) {
        const res = await createAxiosInstance().post('api/digital_timer', timer);
        return res.data;
    },

    /**
     * 
     * @param {number} id Digital timer id to delete.
     * @returns {Promise<void>}
     */
    async delete(id) {
        await createAxiosInstance().delete(`api/digital_timer/${id}`);
    },
};

export default digitalTimerApi;